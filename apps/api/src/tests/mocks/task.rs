use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    sync::Arc,
};

use async_trait::async_trait;
use chrono_tz::Tz;
use tokio::sync::RwLock;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error, Result},
    },
    tag::types::{TagID, TagModel},
    task::{
        repo::TaskRepository,
        types::{
            TaskID, TaskModel,
            repo::{CreateModel, QueryOpts, TaskState, UpdateModel},
        },
    },
    tests::{helpers::get_today_date_pg, mocks::MockDB},
    user::types::UserID,
};

#[derive(Clone)]
pub struct MockTaskRepository {
    users: Arc<RwLock<HashSet<UserID>>>,
    tasks: MockDB<(TaskID, UserID), TaskModel>,
    tags: Arc<RwLock<HashSet<(TagID, UserID)>>>,
    task_tags: MockDB<(TaskID, UserID), Vec<TagModel>>,

    error: Option<Error>,
    user_tz: Tz,
    last_filter: Arc<RwLock<Option<QueryOpts>>>,
    last_single_tag: Arc<RwLock<Option<TagID>>>,
    last_multi_tag: Arc<RwLock<Option<Vec<TagID>>>>,
}

impl MockTaskRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashSet::new())),
            tasks: Arc::new(RwLock::new(HashMap::new())),
            tags: Arc::new(RwLock::new(HashSet::new())),
            task_tags: Arc::new(RwLock::new(HashMap::new())),
            error: None,
            user_tz: Tz::America__Chicago,
            last_filter: Arc::new(RwLock::new(None)),
            last_single_tag: Arc::new(RwLock::new(None)),
            last_multi_tag: Arc::new(RwLock::new(None)),
        }
    }

    pub fn new_backend_error() -> Self {
        let mut mock = Self::new();
        mock.error = Some(Error::Backend("mock backend error".to_string()));

        mock
    }

    pub fn new_programming_error() -> Self {
        let mut mock = Self::new();
        mock.error = Some(Error::Programming("mock programming error".to_string()));

        mock
    }

    pub fn tz(&self) -> Tz {
        self.user_tz
    }

    pub async fn add_user(&self) -> UserID {
        let user_id = UserID::new_v4();

        let mut users = self.users.write().await;
        assert!(users.insert(user_id));

        user_id
    }

    pub async fn add_tag(&self, user_id: UserID) -> TagID {
        let tag_id = TagID::new_v4();

        let mut tags = self.tags.write().await;
        assert!(tags.insert((tag_id, user_id)));

        tag_id
    }

    pub async fn get_last_filter(&self) -> QueryOpts {
        let mut last_filter = self.last_filter.write().await;
        let res = last_filter.take();

        res.unwrap()
    }

    pub async fn get_last_single_tag(&self) -> TagID {
        let mut last_single_tag = self.last_single_tag.write().await;
        let res = last_single_tag.take();

        res.unwrap()
    }

    pub async fn get_last_multi_tag(&self) -> Vec<TagID> {
        let mut last_multi_tag = self.last_multi_tag.write().await;
        let res = last_multi_tag.take();

        res.unwrap()
    }
}

#[async_trait]
impl TaskRepository for MockTaskRepository {
    async fn task_exists(&self, id: TaskID, user_id: UserID) -> Result<bool> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.tasks.read().await;

        Ok(repo.contains_key(&(id, user_id)))
    }

    async fn list(&self, _user_id: UserID, query: Option<QueryOpts>) -> Result<Vec<TaskModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        if let Some(query) = query {
            let mut last_filter = self.last_filter.write().await;
            *last_filter = Some(query);
        }

        Ok(vec![])
    }

    async fn create(&self, user_id: UserID, create_model: CreateModel) -> Result<TaskModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.tasks.write().await;
        let user_repo = self.users.read().await;

        if !user_repo.contains(&user_id) {
            return Err(Error::Constraint(ConstraintViolation::MissingUser));
        }

        let created_at = get_today_date_pg();
        let task = TaskModel {
            id: TaskID::new_v4(),
            title: create_model.title,
            notes: create_model.notes,
            start_dt: create_model.start,
            has_time: create_model.has_time,
            deadline: create_model.deadline,
            position_key: create_model.position_key,
            completed_at: None,
            deleted_at: None,
            updated_at: created_at,
            created_at,
            created_by: user_id,
        };
        repo.insert((task.id, user_id), task.clone());

        Ok(repo.get(&(task.id, user_id)).unwrap().clone())
    }

    async fn get(&self, id: TaskID, user_id: UserID) -> Result<TaskState<TaskModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.tasks.read().await;

        let task_opt = repo.get(&(id, user_id));
        if let Some(task) = task_opt {
            if task.deleted_at.is_some() {
                Ok(TaskState::Deleted(task.clone()))
            } else {
                Ok(TaskState::Existing(task.clone()))
            }
        } else {
            Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )))
        }
    }

    async fn update(
        &self,
        id: TaskID,
        user_id: UserID,
        update_model: UpdateModel,
    ) -> Result<TaskModel> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.tasks.write().await;

        let task_opt = repo.get(&(id, user_id));
        if task_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }
        let mut task = task_opt.unwrap().clone();

        let mut changed = false;
        let now = get_today_date_pg();
        if let Some(title) = update_model.title
            && title != task.title
        {
            task.title = title;
            changed = true;
        }
        if let Some(notes) = update_model.notes
            && notes != task.notes
        {
            task.notes = notes;
            changed = true;
        }
        if let Some(start) = update_model.start
            && start != task.start_dt
        {
            task.start_dt = start;
            changed = true;
        }
        if let Some(has_time) = update_model.has_time
            && has_time != task.has_time
        {
            task.has_time = has_time;
            changed = true;
        }
        if let Some(deadline) = update_model.deadline
            && deadline != task.deadline
        {
            task.deadline = deadline;
            changed = true;
        }
        if let Some(position_key) = update_model.position_key
            && position_key != task.position_key
        {
            task.position_key = position_key;
            changed = true;
        }
        if let Some(completed) = update_model.completed
            && completed != task.completed_at.is_some()
        {
            if completed {
                task.completed_at = Some(now);
            } else {
                task.completed_at = None;
            }
            changed = true;
        }
        if let Some(deleted) = update_model.deleted
            && deleted != task.deleted_at.is_some()
        {
            if deleted {
                task.deleted_at = Some(now);
            } else {
                task.deleted_at = None;
            }
            changed = true;
        }

        if !changed {
            return Ok(task);
        }

        task.updated_at = now;
        repo.insert((id, user_id), task.clone());

        Ok(task)
    }

    async fn delete(&self, id: TaskID, user_id: UserID) -> Result<()> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut repo = self.tasks.write().await;

        let task_opt = repo.get(&(id, user_id));
        if task_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        repo.remove(&(id, user_id)).unwrap();

        Ok(())
    }

    async fn list_task_tags(
        &self,
        ids: Vec<TaskID>,
        user_id: UserID,
    ) -> Result<HashMap<TaskID, Vec<TagModel>>> {
        let task_tags_store = self.task_tags.read().await;

        let mut task_tags = HashMap::new();

        for task_id in ids {
            task_tags.insert(
                task_id,
                task_tags_store
                    .get(&(task_id, user_id))
                    .cloned()
                    .unwrap_or_default(),
            );
        }

        Ok(task_tags)
    }

    async fn list_tags(&self, id: TaskID, user_id: UserID) -> Result<Vec<TagModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let repo = self.tasks.read().await;
        let tag_repo = self.task_tags.read().await;

        let task_opt = repo.get(&(id, user_id));
        if task_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        let tag_list_opt = tag_repo.get(&(id, user_id));
        if let Some(tags) = tag_list_opt {
            Ok(tags.clone())
        } else {
            Ok(vec![])
        }
    }

    async fn add_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut last_tag = self.last_single_tag.write().await;
        *last_tag = Some(tag_id);

        let tags = self.tags.read().await;
        let repo = self.tasks.read().await;
        let mut tag_repo = self.task_tags.write().await;

        let task_opt = repo.get(&(id, user_id));
        if task_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }
        if !tags.contains(&(tag_id, user_id)) {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Tag,
            )));
        }

        match tag_repo.entry((id, user_id)) {
            Entry::Occupied(mut o) => {
                if !o.get().iter().any(|tag| tag.id == tag_id) {
                    o.get_mut().push(TagModel {
                        id: tag_id,
                        ..Default::default()
                    });
                }
            }
            Entry::Vacant(v) => {
                v.insert(vec![TagModel {
                    id: tag_id,
                    ..Default::default()
                }]);
            }
        }

        Ok(())
    }

    async fn remove_tag(&self, id: TaskID, user_id: UserID, tag_id: TagID) -> Result<()> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut last_tag = self.last_single_tag.write().await;
        *last_tag = Some(tag_id);

        let repo = self.tasks.read().await;
        let mut tag_repo = self.task_tags.write().await;

        let task_opt = repo.get(&(id, user_id));
        if task_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }

        match tag_repo.entry((id, user_id)) {
            Entry::Occupied(mut o) => {
                o.insert(
                    o.get()
                        .iter()
                        .filter(|tag| tag.id != tag_id)
                        .map(|tag| tag.to_owned())
                        .collect(),
                );
            }
            Entry::Vacant(_) => (),
        }

        Ok(())
    }

    async fn set_tags(
        &self,
        id: TaskID,
        user_id: UserID,
        tag_ids: Vec<TagID>,
    ) -> Result<Vec<TagModel>> {
        if let Some(err) = &self.error {
            return Err(err.clone());
        }
        let mut last_tags = self.last_multi_tag.write().await;
        *last_tags = Some(tag_ids.clone());

        let tags = self.tags.read().await;
        let repo = self.tasks.read().await;
        let mut tag_repo = self.task_tags.write().await;

        let task_opt = repo.get(&(id, user_id));
        if task_opt.is_none() {
            return Err(Error::Constraint(ConstraintViolation::NotFound(
                Resource::Task,
            )));
        }
        for tag_id in tag_ids.iter() {
            if !tags.contains(&(*tag_id, user_id)) {
                return Err(Error::Constraint(ConstraintViolation::NotFound(
                    Resource::Tag,
                )));
            }
        }

        let tags: Vec<TagModel> = tag_ids
            .iter()
            .map(|tag_id| TagModel {
                id: *tag_id,
                ..Default::default()
            })
            .collect();
        match tag_repo.entry((id, user_id)) {
            Entry::Occupied(mut o) => {
                let current: HashSet<TagID> = o.get().iter().map(|tag| tag.id).collect();
                let desired: HashSet<TagID> = tag_ids.into_iter().collect();
                if current == desired {
                    return Ok(o.get().to_vec());
                }

                o.insert(tags.clone());
            }
            Entry::Vacant(v) => {
                v.insert(tags.clone());
            }
        }

        Ok(tags)
    }
}

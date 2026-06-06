export type Task = {
  id: string;

  title: string;
  notes?: string;
  start?: Date;
  deadline?: Date;
  tags?: Tag[];

  completed_at?: Date;
  deleted_at?: Date;

  created_at: Date;
  updated_at: Date;
};
export type TaskCreate = Pick<Task, 'title' | 'notes' | 'start' | 'deadline' | 'tags'>;

export type Tag = {
  id: string;

  label: string;
  category?: string;

  created_at: Date;
  updated_at: Date;
};
export type TagCreate = Pick<Tag, 'label' | 'category'>;

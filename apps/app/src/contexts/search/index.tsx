import {
  type PropsWithChildren,
  createContext,
  useContext,
  useState,
} from 'react';

type SearchContextType = {
  visible: boolean;
  openSearch: () => void;
  closeSearch: () => void;
};

const SearchContext = createContext<SearchContextType | undefined>(undefined);

type SearchProviderProps = PropsWithChildren;

export default function SearchProvider({ children }: SearchProviderProps) {
  const [visible, setVisible] = useState(false);

  function openSearch() {
    setVisible(true);
  }

  function closeSearch() {
    setVisible(false);
  }

  return (
    <SearchContext
      value={{
        visible,
        openSearch,
        closeSearch,
      }}
    >
      {children}
    </SearchContext>
  );
}

export function useSearchContext() {
  const ctx = useContext(SearchContext);
  if (!ctx) {
    throw new Error('Search Context must be used inside Search Provider');
  }

  return ctx;
}

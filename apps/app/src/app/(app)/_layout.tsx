import AppShell from '@components/app-shell';
import SearchModal from '@components/modals/search-modal';

export default function RootLayout() {
  return (
    <>
      <AppShell />

      <SearchModal />
    </>
  );
}

import type { ReactNode } from 'react'
import { AppSidebar } from '../components/AppSidebar'
import styles from './AppLayout.module.css'

export function AppLayout({ children }: { children: ReactNode }) {
  return (
    <div className={styles.shell}>
      <AppSidebar />
      <div className={styles.content}>{children}</div>
    </div>
  )
}

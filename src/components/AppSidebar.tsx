import styles from './AppSidebar.module.css'

const NAV_SECTIONS = [
  { id: 'library', label: 'Library' },
  { id: 'crates', label: 'Crates' },
  { id: 'tracking', label: 'Tracking' },
  { id: 'settings', label: 'Settings' },
] as const

export function AppSidebar() {
  return (
    <aside className={styles.sidebar}>
      <div className={styles.brand}>
        <span className={styles['brand-name']}>Colkt</span>
        <span className={styles['brand-tagline']}>Personal media library</span>
      </div>
      <nav aria-label="Main navigation">
        <ul className={styles['nav-list']}>
          {NAV_SECTIONS.map((section) => (
            <li
              key={section.id}
              className={styles['nav-item']}
              aria-disabled="true"
            >
              <span className={styles['nav-label']}>{section.label}</span>
              <span className={styles['nav-status']}>Soon</span>
            </li>
          ))}
        </ul>
      </nav>
    </aside>
  )
}

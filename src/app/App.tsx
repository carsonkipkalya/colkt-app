import { AppLayout } from '../layouts/AppLayout'
import styles from './App.module.css'

export function App() {
  return (
    <AppLayout>
      <section className={styles.welcome}>
        <h1 className={styles.title}>Colkt</h1>
        <p className={styles.subtitle}>
          Your private, offline-first personal media library.
        </p>
        <p className={styles.note}>
          Library features are under development.
        </p>
      </section>
    </AppLayout>
  )
}

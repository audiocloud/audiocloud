import {
  HomeIcon,
  Cog6ToothIcon,
  QueueListIcon,
  CircleStackIcon,
  ServerStackIcon,
} from '@heroicons/react/24/outline'

export const pages = [
  { name: 'Dashboard',      href: '/',              icon: HomeIcon },
  { name: 'Audio Engines',  href: '/audio-engines', icon: Cog6ToothIcon },
  { name: 'Devices',        href: '/devices',       icon: ServerStackIcon },
  { name: 'Media',          href: '/media',         icon: CircleStackIcon },
  { name: 'Tasks',          href: '/tasks',         icon: QueueListIcon },
]
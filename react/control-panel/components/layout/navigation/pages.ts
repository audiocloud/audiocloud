import {
  HomeIcon,
  BuildingOfficeIcon,
  Cog6ToothIcon,
  QueueListIcon,
  CircleStackIcon,
  ServerStackIcon,
} from '@heroicons/react/24/outline'

export const pages = [
  { name: 'Dashboard',      href: '/',              icon: HomeIcon },
  { name: 'Domain',         href: '/domain',        icon: BuildingOfficeIcon },
  { name: 'Audio Engines',  href: '/audio-engines', icon: Cog6ToothIcon },
  { name: 'Instances',      href: '/instances',     icon: ServerStackIcon },
  { name: 'Media',          href: '/media',         icon: CircleStackIcon },
  { name: 'Tasks',          href: '/tasks',         icon: QueueListIcon },
]
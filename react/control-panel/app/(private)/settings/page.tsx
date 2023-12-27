'use client'

import React from 'react'
import TopBar from '@/components/layout/page/TopBar/TopBar'

const DashboardPage: React.FC = () => {


  return (
    <div className='w-full'>

      <TopBar title='Settings' subtitle='<account>'>
      </TopBar>

      <div className='p-3'>
        What settings do we want here?
        <ul className='list-disc'>
          <li className='ml-8'>password change</li>
        </ul>
      </div>


    </div>
  )
}

export default DashboardPage
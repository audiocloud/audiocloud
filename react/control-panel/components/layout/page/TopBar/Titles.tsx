import React from 'react'

type Props = {
  title: string,
  subtitle?: string
}

const Titles: React.FC<Props> = ({ title, subtitle }) => {
  return (
    <div className='flex flex-col md:flex-row justify-center items-start md:items-center md:gap-4'>
      { title    && <h1 className='text-lg font-medium sm:truncate'>{ title }</h1> }
      { subtitle && <h2 className='text-sm text-slate-400 sm:truncate'>{ subtitle }</h2> }
    </div>
  )
}

export default Titles
import React from 'react'
import { Instance } from '@/types'
import { PlayIcon, StopIcon, BackwardIcon } from '@heroicons/react/20/solid'
import { InstancePlayState } from '@/utils/domainClient/types'
import classNames from 'classnames'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  instance: Instance,
  playState: InstancePlayState | 'unknown',
  handlePlay: (playState: boolean) => void,
  className: string
}

const Transport: React.FC<Props> = ({ instance, playState, handlePlay, className }) => {
  return (
    <Widget title={'Transport'} className={`${className} text-slate-600 text-sm`}>

      <div className='flex justify-between gap-2 border-b border-slate-200 pb-2 mb-2'>
        <div>Play State</div>
        <div className='flex items-center gap-2'>
          <div className={classNames('w-2.5 h-2.5 rounded-full',
            playState === 'unknown' && 'bg-gray-500',
            playState === 'busy' && 'bg-red-500',
            playState === 'idle' && 'bg-blue-500',
            playState === 'rewinding' && 'bg-amber-500',
            typeof playState === 'object' && 'playing' in playState && 'bg-emerald-600',
          )}></div>
          <div>{ typeof playState === 'object' && 'playing' in playState ? 'playing' : JSON.stringify(playState).split('"') }</div>
        </div>
      </div>
      
      <div className='flex flex-col items-center gap-2'>

        <div className='flex items-center gap-2'>

          <button
            type='button'
            onClick={() => console.log('Rewind clicked!')}
            className='pl-3 pr-4 py-2 w-fit bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border border-slate-300 hover:border-slate-400 active:border-slate-500 rounded-md text-slate-700 text-xs shadow-sm flex justify-between items-center'
          >
            <BackwardIcon className="h-4 w-4 mr-2" aria-hidden="false" />
            <span>Rewind</span>
          </button>

          <button
            type='button'
            onClick={() => handlePlay(true)}
            className='pl-3 pr-4 py-2 w-fit bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border border-slate-300 hover:border-slate-400 active:border-slate-500 rounded-md text-slate-700 text-xs shadow-sm flex justify-between items-center'
          >
            <PlayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
            <span>Play</span>
          </button>

          <button
            type='button'
            onClick={() => handlePlay(false)}
            className='pl-3 pr-4 py-2 w-fit bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border border-slate-300 hover:border-slate-400 active:border-slate-500 rounded-md text-slate-700 text-xs shadow-sm flex justify-between items-center'
          >
            <StopIcon className="h-4 w-4 mr-2" aria-hidden="false" />
            <span>Stop</span>
          </button>

        </div>

      </div>

      <div className='flex justify-between gap-2 border-y border-slate-200 py-2 my-2'>
        <div>Test file loaded</div>
        <div>n/a</div>
      </div>

      <input type='search' placeholder='Search...'/>

    </Widget>
  )
}

export default Transport
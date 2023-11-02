import React, { DetailedHTMLProps, ButtonHTMLAttributes } from 'react'
import { cn } from '@/lib/utils'

type Props = {
  icon: JSX.Element,
  label: string,
  onClickHandler: () => void
}

const TopBarButton: React.FC<Props & DetailedHTMLProps<ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>> = ({ icon, label, onClickHandler, disabled }) => {
  return (
    <button
      type='button'
      onClick={() => onClickHandler()}
      className={cn(
        'pl-3 pr-4 py-2 flex justify-between items-center text-xs shadow-sm rounded-md border',
        !disabled && 'bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border-slate-300 hover:border-slate-400 active:border-slate-500 text-slate-700',
        disabled && 'bg-slate-100 border-slate-200 text-slate-500'
      )}
      disabled={disabled}
    >
      { icon }
      { label }
    </button>
  )
}

export default TopBarButton
import React, { DetailedHTMLProps, ButtonHTMLAttributes } from 'react'

type Props = {
  onClickHandler: () => void,
  icon: JSX.Element
}

const MaintenanceActionButton: React.FC<Props & DetailedHTMLProps<ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>> = ({ onClickHandler, icon }) => {
  return (
    <button
      type='button'
      className="rounded-md flex justify-center items-center p-1 text-amber-600 hover:text-amber-300 border border-amber-500 hover:border-amber-700 bg-amber-300 hover:bg-amber-600 active:bg-amber-700"
      onClick={() => onClickHandler()}
    >
      { icon }
    </button>
  )
}

export default MaintenanceActionButton

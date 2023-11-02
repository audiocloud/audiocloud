import React, { DetailedHTMLProps, ButtonHTMLAttributes } from 'react'

type Props = {
  onClickHandler: () => void,
  icon: JSX.Element
}

const AlertActionButton: React.FC<Props & DetailedHTMLProps<ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>> = ({ onClickHandler, icon }) => {
  return (
    <button
      type='button'
      className="rounded-md flex justify-center items-center p-1 text-red-600 hover:text-red-200 border border-red-400 hover:border-red-600 bg-red-300 hover:bg-red-500 active:bg-red-600"
      onClick={() => onClickHandler()}
    >
      { icon }
    </button>
  )
}

export default AlertActionButton
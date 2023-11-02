import React, { Fragment, useRef } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import { ExclamationTriangleIcon, TrashIcon, XMarkIcon } from '@heroicons/react/24/outline'

type Props = {
  media_id: string,
  open: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const DeleteConfirmation: React.FC<Props> = ({ media_id, open, setOpen }) => {

  const cancelButtonRef = useRef(null)

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog as="div" className="relative z-10" initialFocus={cancelButtonRef} onClose={setOpen}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                
                {/* CONTENT*/}
                <div className="mb-10">

                  {/* HEADER */}
                  <div className='flex justify-center sm:justify-start items-center gap-3 mb-6'>
                    <div className="flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-md bg-red-100 border border-red-300 sm:mx-0 sm:h-10 sm:w-10">
                      <ExclamationTriangleIcon className="h-6 w-6 text-red-600" aria-hidden="false" />
                    </div>
                    <Dialog.Title as="h3" className="text-lg font-medium leading-6 text-slate-700">
                      Delete Media Item
                    </Dialog.Title>
                  </div>

                  {/* BODY */}
                  <div className="mt-3 text-center sm:mt-0 sm:text-left">
                    <div className="mt-2 text-sm text-gray-500 flex flex-col gap-1">

                      <div className="text-sm text-gray-500">Are you sure you want to delete this media item:</div>
                      <div className="text-lg text-gray-800">{ media_id }</div>

                    </div>
                  </div>

                </div>

                {/* ACTIONS */}
                <div className="mt-6 sm:mt-4 flex flex-col sm:flex-row-reverse gap-2">
                  <button
                    type="button"
                    className="inline-flex w-full justify-center items-center bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border border-slate-300 hover:border-slate-400 active:border-slate-500 text-slate-700 rounded-md pl-2 pr-3 py-2 text-base font-medium shadow-sm sm:w-auto sm:text-sm"
                    onClick={() => setOpen(false)}
                  >
                    <TrashIcon className='text-slate-500 w-5 h-5 mr-1' aria-hidden='false'/>Delete
                  </button>
                  <button
                    type="button"
                    className="inline-flex w-full justify-center items-center bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border border-slate-300 hover:border-slate-400 active:border-slate-500 text-slate-700 rounded-md pl-2 pr-3 py-2 text-base font-medium shadow-sm sm:w-auto sm:text-sm"
                    onClick={() => setOpen(false)}
                  >
                    <XMarkIcon className='text-slate-500 w-5 h-5 mr-1' aria-hidden='false' />Cancel
                  </button>
                </div>

              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  )
}

export default DeleteConfirmation
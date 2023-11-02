import React, { Fragment, useState } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import { ExclamationTriangleIcon, XMarkIcon, CheckIcon } from '@heroicons/react/24/outline'

type Props = {
  engine_id: string,
  open: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const NewMaintenance: React.FC<Props> = ({ engine_id, open, setOpen }) => {

  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [start, setStart] = useState('')
  const [end, setEnd] = useState('')

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog as="div" className="relative z-10" onClose={() => console.log()}>
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
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pt-6 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-2xl sm:p-6">

                {/* CLOSE ICON */}
                <div className="absolute top-0 right-0 hidden pt-4 pr-4 sm:block">
                  <button
                    type="button"
                    className="rounded-md bg-white text-gray-400 hover:text-gray-500"
                    onClick={() => setOpen(false)}
                  >
                    <span className="sr-only">Close</span>
                    <XMarkIcon className="h-6 w-6" aria-hidden="false" />
                  </button>
                </div>

                {/* CONTENT */}
                <div className="mb-10">
                  
                  {/* HEADER */}
                  <div className='flex justify-center sm:justify-start items-center gap-3 mb-6'>
                    <div className="flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-md bg-yellow-100 border border-yellow-300 sm:mx-0 sm:h-10 sm:w-10">
                      <ExclamationTriangleIcon className="h-6 w-6 text-yellow-600" aria-hidden="false" />
                    </div>
                    <Dialog.Title as="h3" className="text-lg font-medium leading-6 text-slate-700">
                      New Maintenance
                    </Dialog.Title>
                  </div>

                  {/* BODY */}
                  <div className="mt-3 text-center sm:mt-0 sm:text-left">
                    <div className="mt-2 text-sm text-gray-500 flex flex-col gap-4">

                      <div>
                        <label htmlFor='engine-id' className='text-xs'>Engine ID</label>
                        <div id='engine-id' className='text-gray-800 font-medium text-lg'>{ engine_id }</div>
                      </div>

                      <div>
                        <label htmlFor='title' className='text-xs'>Title</label>
                        <input
                          id='title'
                          type='text'
                          className='block w-full px-2 py-1 bg-slate-50 border border-slate-300 rounded-md text-sm text-gray-600'
                          value={title}
                          onChange={(e) => setTitle(e.target.value)}
                        />
                      </div>

                      <div>
                        <label htmlFor='description' className='text-xs'>Description</label>
                        <textarea
                          id='description'
                          className='block w-full px-2 py-1 bg-slate-50 border border-slate-300 rounded-md text-sm text-gray-600'
                          value={description}
                          onChange={(e) => setDescription(e.target.value)}
                        />
                      </div>

                      <div className='flex justify-start items-center gap-5'>
                        <div>
                          <label htmlFor='start-time' className='text-xs'>Start time</label>
                          <input
                            id='start-time'
                            type='datetime-local'
                            className='block bg-slate-50 border-slate-300 rounded-md text-gray-600 text-sm'
                            value={ start }
                            onChange={(e) => console.log(e.target.value)}
                          />
                        </div>

                        <div>
                          <label htmlFor='end-time' className='text-xs'>End time</label>
                          <input
                            id='end-time'
                            type='datetime-local'
                            className='block bg-slate-50 border-slate-300 rounded-md text-gray-600 text-sm'
                            value={ end }
                            onChange={(e) => console.log(e.target.value)}
                          />
                        </div>
                      </div>

                    </div>

                  </div>
                </div>

                {/* ACTION BUTTONS */}
                <div className="mt-6 sm:mt-4 flex flex-col sm:flex-row-reverse gap-2">
                  <button
                    type="button"
                    className="inline-flex w-full justify-center items-center bg-slate-200 hover:bg-slate-300 active:bg-slate-400 border border-slate-300 hover:border-slate-400 active:border-slate-500 text-slate-700 rounded-md pl-2 pr-3 py-2 text-base font-medium shadow-sm sm:w-auto sm:text-sm"
                    onClick={() => setOpen(false)}
                  >
                    <CheckIcon className='text-slate-500 w-5 h-5 mr-1' aria-hidden='false' />Create new
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

export default NewMaintenance
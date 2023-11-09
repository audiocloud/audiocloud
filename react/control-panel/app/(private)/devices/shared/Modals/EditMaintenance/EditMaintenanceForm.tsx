'use client'

import React, { useState } from 'react'
import { useForm } from 'react-hook-form'
import * as z from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { PencilSquareIcon } from '@heroicons/react/24/outline'
import { isBefore } from 'date-fns'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { NewDeviceMaintenanceSchema } from '@/types/zodSchemas'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Textarea } from '@/components/ui/textarea'
import { IDeviceMaintenance } from '@/types'

type Props = {
  maintenance: IDeviceMaintenance,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const EditMaintenanceForm: React.FC<Props> = ({ maintenance, setOpen }) => {

  const form = useForm<z.infer<typeof NewDeviceMaintenanceSchema>>({
    resolver: zodResolver(NewDeviceMaintenanceSchema),
    defaultValues: {
      device_id: maintenance.device_id,
      title: maintenance.data.header,
      description: maintenance.data.body,
      startTime: new Date(maintenance.data.start).toLocaleString(), // TO-DO: this does not work
      endTime: maintenance.data.end ? new Date(maintenance.data.end).toLocaleString() : ''  // TO-DO: this does not work
    }
  })

  const onSubmit = (values: z.infer<typeof NewDeviceMaintenanceSchema>) => {
    try {
      console.log('Edit maintenance!')
      console.table(values)
      setOpen(false)
    } catch (error) {
      setError(error as string) // TO-DO: error type handling
    }
  }
  const [error, setError] = useState('')

  return (
    <Form {...form}>
      <form
        className='space-y-6'
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <FormField
          control={form.control}
          name='device_id'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Device ID</FormLabel>
              <FormControl>
                <Input {...field} disabled={true} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name='title'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Title</FormLabel>
              <FormControl>
                <Input type='text' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name='description'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Description</FormLabel>
              <FormControl>
                <Textarea {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <div className='flex justify-between items-center'>
          <FormField
            control={form.control}
            name='startTime'
            render={({ field }) => (
              <FormItem>
                <FormLabel>Start</FormLabel>
                <FormControl>
                  <Input type='datetime-local' {...field} disabled={isBefore(new Date(field.value), new Date())}/>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name='endTime'
            render={({ field }) => (
              <FormItem>
                <FormLabel>End</FormLabel>
                <FormControl>
                  <Input type='datetime-local' {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <div className='text-foreground-secondary text-sm'>Last edit: { new Date(maintenance.data.updated_at).toLocaleString() }</div>

        <div className='pt-3 flex justify-center items-center gap-4'>

          <Button
            type='button'
            variant='objectActionButton'
            size='default'
            className='flex justify-center items-center gap-2'
            onClick={() => setOpen(false)}
          >
            <span>Cancel</span>
          </Button>

          <Button
            type='submit'
            variant='warning'
            size='default'
            className='flex justify-center items-center gap-2'
          >
            <PencilSquareIcon className='w-5 h-5' aria-hidden="false" />
            <span>Update</span>
          </Button>

        </div>
        
      </form>

      { error && (
        <Alert
          variant='destructive'
          className='mt-5'
        >
          <AlertTitle>Action failed.</AlertTitle>
          <AlertDescription>{ error }</AlertDescription>
        </Alert>
      )}

    </Form>
  )
}

export default EditMaintenanceForm
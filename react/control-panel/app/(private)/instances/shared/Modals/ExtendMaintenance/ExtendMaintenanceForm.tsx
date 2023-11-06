'use client'

import React, { useState } from 'react'
import { useForm } from 'react-hook-form'
import * as z from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { ClockIcon } from '@heroicons/react/24/outline'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { NewInstanceMaintenanceSchema } from '@/types/zodSchemas'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { IInstanceMaintenance } from '@/types'

type Props = {
  maintenance: IInstanceMaintenance,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const ExtendMaintenanceForm: React.FC<Props> = ({ maintenance, setOpen }) => {

  const form = useForm<z.infer<typeof NewInstanceMaintenanceSchema>>({
    resolver: zodResolver(NewInstanceMaintenanceSchema),
    defaultValues: {
      instance_id: maintenance.instance_id,
      title: maintenance.data.header,
      description: maintenance.data.body,
      startTime: new Date(maintenance.data.start).toLocaleString(), // TO-DO: this does not work
      endTime: maintenance.data.end ? new Date(maintenance.data.end).toLocaleString() : ''  // TO-DO: this does not work
    }
  })

  const onSubmit = (values: z.infer<typeof NewInstanceMaintenanceSchema>) => {
    try {
      console.log('Extend maintenance!')
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
          name='instance_id'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Instance ID</FormLabel>
              <FormControl>
                <Input {...field} disabled={true} />
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
                <Input type='datetime-local' className='w-1/2 ' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

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
            <ClockIcon className='w-5 h-5' aria-hidden="false" />
            <span>Extend</span>
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

export default ExtendMaintenanceForm
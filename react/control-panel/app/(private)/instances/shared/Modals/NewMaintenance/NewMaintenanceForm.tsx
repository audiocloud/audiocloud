'use client'

import React, { useState } from 'react'
import { useForm } from 'react-hook-form'
import * as z from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { CheckIcon } from '@heroicons/react/24/outline'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { NewInstanceMaintenanceSchema } from '@/types/zodSchemas'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Textarea } from '@/components/ui/textarea'

type Props = {
  instance_id: string,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const NewMaintenanceForm: React.FC<Props> = ({ instance_id, setOpen }) => {

  const form = useForm<z.infer<typeof NewInstanceMaintenanceSchema>>({
    resolver: zodResolver(NewInstanceMaintenanceSchema),
    defaultValues: {
      instance_id,
      title: '',
      description: '',
      startTime: '',
      endTime: ''
    }
  })

  const onSubmit = (values: z.infer<typeof NewInstanceMaintenanceSchema>) => {
    try {
      console.log('New maintenance!')
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
                  <Input type='datetime-local' {...field} />
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
            variant='emerald'
            size='default'
            className='flex justify-center items-center gap-2'
          >
            <CheckIcon className='w-5 h-5' aria-hidden="false" />
            <span>Confirm Maintenance</span>
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

export default NewMaintenanceForm
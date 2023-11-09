'use client'

import React, { useState } from 'react'
import { useForm } from 'react-hook-form'
import * as z from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import useAudioCloudAuth from '@/hooks/useAudioCloudAuth'
import { LoginFormSchema } from '@/types/zodSchemas'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'

const SignInForm: React.FC = () => {

  const form = useForm<z.infer<typeof LoginFormSchema>>({
    resolver: zodResolver(LoginFormSchema),
    defaultValues: {
      id: '',
      password: ''
    }
  })

  const onSubmit = (values: z.infer<typeof LoginFormSchema>) => {
    login(values.id, values.password, setError)
  }
  const [error, setError] = useState('')

  const { login } = useAudioCloudAuth()

  return (
    <Form {...form}>
      <form
        className='space-y-6'
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <FormField
          control={form.control}
          name='id'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Admin ID</FormLabel>
              <FormControl>
                <Input {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name='password'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Password</FormLabel>
              <FormControl>
                <Input type='password' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormDescription className='w-full text-center'>
          <span>Can't login? Use </span>
          <a href='https://docs.audiocloud.io/' className='underline' target='_blank'>AudioCloud CLI</a>
          <span> to get/reset your admin account.</span>
        </FormDescription>

        <div className='pt-3 w-full text-center'>
          <Button type='submit' size='lg' variant='outline'>Login</Button>
        </div>
        
      </form>

      { error && (
        <Alert
          variant='destructive'
          className='mt-5'
        >
          <AlertTitle>Sign in failed.</AlertTitle>
          <AlertDescription>{ error }</AlertDescription>
        </Alert>
      )}

    </Form>
  )
}

export default SignInForm
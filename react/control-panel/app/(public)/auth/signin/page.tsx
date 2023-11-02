import React from 'react'
import SignInForm from './SignInForm'

const SignInPage: React.FC = () => {

  return (

    <div className='w-screen h-screen flex justify-center'>
      
      <div className='mt-[10vh] lg:mt-[20vh] w-[400px] flex flex-col duration-200'>

        <h1 className='text-3xl text-center font-bold'>AudioCloud Domain Control</h1>
        <h2 className='mt-3 mb-6 text-xl text-center font-medium uppercase'>Sign In</h2>
        <SignInForm/>

      </div>

    </div>
  )
}

export default SignInPage
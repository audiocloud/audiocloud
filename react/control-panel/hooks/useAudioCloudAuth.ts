'use client'

import { loadEnvVar } from '../utils/loadEnvVar'
import axios, { AxiosRequestConfig } from 'axios'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'

const DOMAIN_SERVER_IP = loadEnvVar(process.env.NEXT_PUBLIC_DOMAIN_SERVER_IP, 'NEXT_PUBLIC_DOMAIN_SERVER_IP')

const useAudioCloudAuth = () => {

  // useCookies needs removeCookie to be the 3rd export or will in fact be setCookie
  const [cookies, setCookie, removeCookie] = useCookies(['token'])
  
  const [id, setId] = useState('')
  const [email, setEmail] = useState('')

  const login = async (id: string, password: string, setError: React.Dispatch<React.SetStateAction<string>>) => {
    try {
      const config: AxiosRequestConfig = {
        headers: { 'Content-Type': 'application/json' },
        withCredentials: true
      }
      const { data } = await axios.post(
        `http://${DOMAIN_SERVER_IP}:7200/api/v1/users/login`,
        { id, password },
        config
      )
      console.log('Data:', data)
      setCookie('token', data.token, { maxAge: 3600, path: '/', sameSite: 'lax' })
    } catch (error) {
      setError((error as any).message) // TO-DO: get error message
      console.log(error)
    }
  }

  const logout = async () => {
    try {
      const config: AxiosRequestConfig = {
        headers: { 'Content-Type': 'application/json' },
        withCredentials: true
      }
      const { status, statusText } = await axios.get(
        `http://${DOMAIN_SERVER_IP}:7200/api/v1/users/logout`,
        config
      )
      if (status === 200) throw Error(`Error: ${status} - ${statusText}`)
      removeCookie('token', { maxAge: 3600, path: '/', sameSite: 'lax' })
    } catch (error) {
      console.log(error)
    }
  }

  const whoAmI = async () => {
    try {
    const config: AxiosRequestConfig = {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${cookies.token}`
      },
      withCredentials: true
    }
      const { status, statusText, data } = await axios.get(
        `http://${DOMAIN_SERVER_IP}:7200/api/v1/users/whoami`,
        config
      )
      console.log('Data:', data)
      if (status !== 200) throw Error(`Error: ${status} - ${statusText}`)
      setId(data.id)
      setEmail(data.email)
    } catch (error) {
      console.log(error)
    }
  }

  useEffect(() => {
    if (cookies.token) whoAmI()
  }, [cookies.token])

  return { id, email, login, logout, whoAmI }
}

export default useAudioCloudAuth
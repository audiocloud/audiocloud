import type { Metadata } from 'next'
import { Jost } from 'next/font/google'
import '../styles/globals.css'
import NextTopLoader from 'nextjs-toploader'
import RouteGuard from '@/components/RouteGuard'
import { cn } from '@/lib/utils'
import { ThemeProvider } from '@/components/theme/ThemeProvider'

export const fontJost = Jost({
  subsets: ['latin'],
  variable: '--font-jost',
  display: 'swap'
})

export const metadata: Metadata = {
  title: 'AC Control Panel',
  description: 'Generated by create next app',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang='en'>
      <body
        className={cn(
          'min-h-screen bg-background font-jost antialiased',
          fontJost.variable
        )}
      >
        <ThemeProvider
          attribute='class'
          defaultTheme='system'
          enableSystem
          disableTransitionOnChange
        >
          <NextTopLoader color='#2563eb' showSpinner={false} height={3} />
          <RouteGuard>
            { children }
          </RouteGuard>
        </ThemeProvider>
      </body>
    </html>
  )
}

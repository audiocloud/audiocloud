import React, { ReactNode } from 'react'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { cn } from '@/lib/utils'

type Props = {
  label: string,
  className: string,
  children: ReactNode,
}

const CustomCard: React.FC<Props> = ({ label, className, children }) => {
  return (
    <Card className={cn('bg-primary-foreground', className)}>
      <CardHeader className='px-3 pt-3 pb-2 font-bold border-b'>{ label }</CardHeader>
      <CardContent className='p-3'>
        { children }
      </CardContent>
    </Card>
  )
}

export default CustomCard
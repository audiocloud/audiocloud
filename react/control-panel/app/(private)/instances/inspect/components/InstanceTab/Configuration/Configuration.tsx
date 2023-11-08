import React from 'react'
import { IInstance } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import NumberInput from '@/components/general/Card/NumberInput'
import CardLine from '@/components/general/Card/CardLine'
import MediaConfigModal from './MediaConfigModal'
import ConfigDropdown from '@/components/general/Card/ConfigDropdown'

type Props = {
  instance: IInstance
}

const ConfigurationContent: React.FC<Props> = ({ instance }) => {

  return (
    <CustomCard label='Configuration' className='w-[400px]'>
      <div className='w-full flex flex-col justify-center items-center gap-2'>
        
        { instance.media_config !== undefined && (
          <CardLine
            label='Media configuration'
            item={<MediaConfigModal originalConfig={instance.media_config} />}
          />
        )}

        { instance.power_config?.idle_shutdown_timeout_ms && (
          <CardLine
            label='Idle shutdown timeout'
            units='ms'
            item={<NumberInput value={instance.power_config.idle_shutdown_timeout_ms}/>}
          />
        )}

        { instance.power_config?.cool_down_delay_ms && (
          <CardLine
            label='Cooldown delay'
            units='ms' 
            item={<NumberInput value={instance.power_config.cool_down_delay_ms} />}
          />
        )}

        { instance.power_config?.warm_up_delay_ms && (
          <CardLine
            label='Warm up delay'
            units='ms'
            item={<NumberInput value={instance.power_config.warm_up_delay_ms} />}
          />
        )}

        <CardLine
          label='Driver attachment'
          item={instance.driver_attachment_url || 'no attachment'}
        />

        <CardLine
          label='Config Dropdown Test'
          item={<ConfigDropdown label='Select' options={[
            { label: 'Option 1', value: '1' },
            { label: 'Option 2', value: '2' },
            { label: 'Option 3', value: '3' }
          ]}/>}
        />

      </div>
    </CustomCard>
  )
}

export default ConfigurationContent
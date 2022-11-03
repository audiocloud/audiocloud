# Flows

## Standalone boot flow

```mermaid
sequenceDiagram
    participant Domain Server
    participant Audio Engine
    participant Instance Driver
    
    Audio Engine->>Domain Server: Register (with ID and static config)
    Note right of Domain Server: Lookup configuration for Audio Engine, merge, mark audio engine as potentially up (half open)
    Domain Server->>Audio Engine: Here is the configuration
    
    loop Healthcheck
        Domain Server->>Audio Engine: Are you alive?
        Audio Engine->>Domain Server: Yes!
        Note right of Domain Server: On failure set audio engine as down
    end
    
    Instance Driver->>Domain Server: Register (with ID and static config)
    Note right of Domain Server: Lookup configuration for Instance Driver, merge, mark instance as potentially up (half open)
    Domain Server->>Instance Driver: Here is the configuration


    loop Healthcheck
        Domain Server->>Instance Driver: Are you alive?
        Instance Driver->>Domain Server: Yes!
        Note right of Domain Server: On failure set instance driver as down
    end
```

## Orchestrated boot flow

```mermaid
sequenceDiagram
    participant Cloud
    participant Domain Server
    participant Audio Engine
    participant Instance Driver
    
    Domain Server->>Cloud: Register (with ID and static config)
    Note right of Cloud: Lookup configuration for Domain Server, merge, mark domain server as potentially up (half open)
    Cloud->>Domain Server: Here is the configuration
    
    loop Healthcheck
        Cloud->>Domain Server: Are you alive?
        Domain Server->>Cloud: Yes!
        Note right of Cloud: On failure set domain server as down
    end
    
    Audio Engine->>Domain Server: Register (with ID and static config)
    Note right of Domain Server: Lookup configuration for Audio Engine, merge, mark audio engine as potentially up (half open)
    Domain Server->>Audio Engine: Here is the configuration
    
    loop Healthcheck
        Domain Server->>Audio Engine: Are you alive?
        Audio Engine->>Domain Server: Yes!
        Note right of Domain Server: On failure set audio engine as down
    end
    
    Instance Driver->>Domain Server: Register (with ID and static config)
    Note right of Domain Server: Lookup configuration for Instance Driver, merge, mark instance as potentially up (half open)
    Domain Server->>Instance Driver: Here is the configuration


    loop Healthcheck
        Domain Server->>Instance Driver: Are you alive?
        Instance Driver->>Domain Server: Yes!
        Note right of Domain Server: On failure set instance driver as down
    end
```

## Standalone task reservation flow

```mermaid
sequenceDiagram
    participant Domain Client
    participant Domain Server
    participant Instance Driver
    participant Audio Engine
    
    Domain Client->>Domain Server: Reserve task with spec
    Note right of Domain Server: Validate task spec for Domain Client, set task pending, update task list
    Domain Server->>Domain Client: Reservation confirmed
    
    loop Propagate task spec
        Domain Server->>Audio Engine: Push spec
        Audio Engine->>Domain Server: Spec accepted
        Note right of Domain Server: On failure set task as failed
        
        Domain Server->>Instance Driver: Push spec
        Instance Driver->>Domain Server: Spec accepted
        Note right of Domain Server: On failure set task as failed
    end
    
    loop Wait for task state
        Domain Client->>Domain Server: Get task state
        Note right of Domain Server: Check task state, if all instances accepted and engine accepted, set task as OK
        Domain Server->>Domain Client: Here is the task state
    end
```

## Play flow

## Render flow

## Stop and render Cancel flow

## Error flow

## Reservation cancel flow

import {TaskPlayState} from '../src/tasks'

describe('parsing of task summary', () => {
    it('parses `waitingForInstances`  correctly', () => {
        const rv = TaskPlayState.parse({
            type: 'waitingForInstances',
            missing: ['a', 'b'],
            waiting: ['c']
        })

        expect(rv.type).toBe('waitingForInstances')
        if (rv.type == 'waitingForInstances') {
            expect(rv.missing).toStrictEqual(['a', 'b'])
            expect(rv.waiting).toStrictEqual(['c'])
        }
    })

    it('parses `waitingForFiles`  correctly', () => {
        const rv = TaskPlayState.parse({
            type: 'waitingForFiles',
            missing: ['a', 'b'],
            downloading: ['c']
        })

        expect(rv.type).toBe('waitingForFiles')
        if (rv.type == 'waitingForFiles') {
            expect(rv.missing).toStrictEqual(['a', 'b'])
            expect(rv.downloading).toStrictEqual(['c'])
        }
    })
});
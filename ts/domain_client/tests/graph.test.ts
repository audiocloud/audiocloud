import {AudioGraphSpec} from '../src/graph'

describe('parsing of audio graph specification', () => {
    it('parses empty specification', () => {
        const spec = AudioGraphSpec.parse({
            sources: {},
            inserts: {},
            busses: {},
        })
    })

    it('parses specification with one source', () => {
        const spec = AudioGraphSpec.parse({
            sources: {
                1: {
                    startAt: 0,
                    sourceUrl: 'https://example.com',
                    numChannels: 2,
                },
            },
            inserts: {},
            busses: {},
        })

        expect(spec.sources).toHaveProperty("1")
        expect(spec.sources[1].sourceUrl).toEqual('https://example.com')
    })
});

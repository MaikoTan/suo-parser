import test from 'ava'

import { plus } from '../index'

test('sync function from native code', (t) => {
    const fixture = 42
    t.is(plus(fixture), fixture + 100)
})

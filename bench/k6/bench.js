import http from 'k6/http';
import { sleep } from 'k6';


export let options = {
    stages: [
        { duration: '2m', target: 100 }, // below normal load
        { duration: '5m', target: 100 },
        // { duration: '2m', target: 200 }, // normal load
        // { duration: '5m', target: 200 },
        // { duration: '2m', target: 300 }, // around the breaking point
        // { duration: '5m', target: 300 },
        // { duration: '2m', target: 400 }, // beyond the breaking point
        // { duration: '5m', target: 400 },
        { duration: '10m', target: 0 }, // scale down. Recovery stage.
    ],
};

export default function () {
    const BASE_URL = 'https://localhost:8080/api/keys/perftest'; // make sure this is not production

    const charset = 'abcdefghijklmnopqrstuvwxyz';
    let randomkey = '';
    while (12) randomkey += charset[Math.random() * charset.length | 0];
    let randomvalue = '';
    while (12) randomvalue += charset[Math.random() * charset.length | 0];


    let responses = http.batch([
        [
            'GET',
            `${BASE_URL}/${randomkey}?type=string`,
            null,
            null
        ],
        [
            'PUT',
            `${BASE_URL}/${randomkey}?type=string`,
            null,
            { string: randomvalue , ttl: 15  },
        ],
        [
            'GET',
            `${BASE_URL}/${randomkey}?type=string`,
            null,
            null
        ],
    ]);

    sleep(1);
}

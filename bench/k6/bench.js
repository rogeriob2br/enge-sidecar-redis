import http from 'k6/http';
import { sleep } from 'k6';


export let options = {
    // stages: [
    //     { duration: '2m', target: 1 }, // below normal load
    //     { duration: '5m', target: 5 },
    //     // { duration: '2m', target: 200 }, // normal load
    //     // { duration: '5m', target: 200 },
    //     // { duration: '2m', target: 300 }, // around the breaking point
    //     // { duration: '5m', target: 300 },
    //     // { duration: '2m', target: 400 }, // beyond the breaking point
    //     // { duration: '5m', target: 400 },
    //     { duration: '10m', target: 0 }, // scale down. Recovery stage.
    // ],
    vus: 1, // 1 user looping for 1 minute
    duration: '1m',
    thresholds: {
        http_req_duration: ['p(99)<1500'], // 99% of requests must complete below 1.5s
    },
};

export default function () {
    const BASE_URL = 'https://127.0.0.1:8080/api/keys/perftest'; // make sure this is not production

    const charset = 'abcdefghijklmnopqrstuvwxyz';
    let randomkey = '';
    while (12) randomkey += charset[Math.random() * charset.length | 0];
    let randomvalue = '';
    while (12) randomvalue += charset[Math.random() * charset.length | 0];
    let req1 = {
        method: 'GET',
        url: `${BASE_URL}/${randomkey}?type=string`,
    };
    console.log(JSON.stringify(req2));
    let req2 = {
        method: 'PUT',
        url: `${BASE_URL}/${randomkey}?type=string`,
        body: {
            string: randomvalue,
            ttl: 15
        },
    };
    console.log(JSON.stringify(req2));
    let req3 = {
        method: 'GET',
        url: `${BASE_URL}/${randomkey}?type=string`,
        params: {
            headers: { 'Content-Type': 'application/json' },
        },
    };
    console.log(JSON.stringify(req2));

    let responses = http.batch([req1, req2, req3]);

    console.log(JSON.stringify(responses[3]));
    // sleep(1);
}

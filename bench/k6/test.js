import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
    http.get('http://localhost:8080/api/keys/perf-test/00059144189?type=hash');
    //sleep(1);
}
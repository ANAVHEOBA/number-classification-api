ab@ab:~/number-classification-api$ curl "http://localhost:8080/api/classify-number?number=371"
{"number":371,"is_prime":false,"is_perfect":false,"properties":["armstrong","odd"],"digit_sum":11,"fun_fact":"371 is a narcissistic number."}ab@ab:~/number-classification-api$ curl "http://localhost:8080/api/classify-number?number=abc"                        curl "http://localhost:8080/api/classify-number?number=abc"
{"number":"abc","error":true}ab@ab:~/number-classification-api$ 

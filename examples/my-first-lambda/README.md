# my-first-lambda
Example Lambda HTTP function in Rust

1. Build the image
```
docker build --build-arg package=the-first-lambda -t the-first-lambda:latest .
```
2. Run the image
```
docker run -rm -p 8080:8080 the-first-lambda:latest
``` 

3. Invoke the function
```
curl -X POST --data '{"version":"2.0","rawQueryString":"name=beeb","requestContext":{"http":{"method":"GET"},"timeEpoch":0}}' http://localhost:8080/2015-03-31/functions/function/invocations
```
# Background

A learning project with this following context - build a toy v8 isolate based compute platform

# Plan

1. web app exposing the apps, init with dummy app
2. expose endpoint to update app ( imagine sqs setup for app events with sns forwarding to this endpoint )

Test and observe with

```
iwr http://localhost:8080/v1/apps

iwr http://localhost:8080/v1/apps/2 -Method PUT -Body '{"id":"2","name":"app2","version":{"major":2,"minor":0,"revision":0,"commit_id":"465456464"},"description":"what app2 does"}' -ContentType 'application/json'

iwr http://localhost:8080/v1/apps

iwr http://localhost:8080/v1/apps/2 -Method PUT -Body '{"id":"2","name":"app2","version":{"major":2,"minor":0,"revision":0,"commit_id":"465456464"},"description":"what app2 does"}' -ContentType 'application/json'

iwr http://localhost:8080/v1/apps
```

3. Pull down javascript file from s3 to run

# Reference

https://omarabid.com/rust-intro
https://deno.land/

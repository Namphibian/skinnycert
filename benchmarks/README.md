# Taurus HTTP(s) Testing.

This project uses the BZT tool which allows me to execute a various tests without getting into the details.
The subfolders config and artifacts are mounted by the docker compose stack to configure tests and review test results.

See the following for more details on how BZT works:
- https://gettaurus.org/docs/Index/


How to run a test:

```bash
docker exec -it taurus  bzt dev-test.yaml
```

This will execute the dev-test.yaml test scenarios.

## Mounted Folder Information:

1. artifacts: This folder will contain test results after the test has run.
2. config: This contains a simple test that can be expanded on.
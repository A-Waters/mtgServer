# Welcome to your CDK TypeScript project

install aws-cdk cli

```bash
npm install -g aws-cdk-lib aws-cdk
```

# deploy
```
cdk synth
cdk deploy
```

# set up aws account access
1. run the following command to configure your aws account
```
$ aws configure
```

2. update the file cdk.json to include your account and region
```
    "env": {
      "account": "YOUR_ACCOUNT_ID",
      "region": "YOUR_REGION"
    }
```

# first time aws account
run the following command to configure your aws account

```
$ cdk bootstrap
$ cdk deploy
```

```
This is a blank project for CDK development with TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template

import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import * as logs from 'aws-cdk-lib/aws-logs'; // Add this import

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);


    // Create an S3 bucket to store the data
    const dataBucket = new s3.Bucket(this, 'CardDataBucket', {
      bucketName: `card-server-data-bucket-${this.account}`, // Replace with your desired bucket name
      removalPolicy: cdk.RemovalPolicy.DESTROY, // Use with caution, only for development

    });

    // Create an S3 bucket to store the Lambda code
    const bucket = new s3.Bucket(this, 'CardServerCodeBucket', {
      bucketName: `card-server-code-bucket-${this.account}`, // Replace with your desired bucket name
      removalPolicy: cdk.RemovalPolicy.DESTROY, // Use with caution, only for development
    });

    // Create the Lambda function
    const rustLambda = new lambda.Function(this, 'CardLambda', {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'bootstrap', // This is the default handler name for Rust Lambdas
      code: lambda.Code.fromBucket(bucket, 'lambda.zip'),
      architecture: lambda.Architecture.X86_64, // Use ARM architecture for better performance and lower cost
      logRetention: logs.RetentionDays.ONE_WEEK, // Add CloudWatch Logs retention
      timeout: cdk.Duration.seconds(60),
    });

    // allow lambda to read from s3
    dataBucket.grantRead(rustLambda);

    // Create an API Gateway
    const api = new apigateway.RestApi(this, 'CardLambdaAPI', {
      restApiName: 'Rust Card Lambda API',
      description: 'API for Rust Card Lambda function',
    });

    // Create a resource and method to integrate the Lambda with API Gateway
    const lambdaIntegration = new apigateway.LambdaIntegration(rustLambda);
    api.root.addMethod('GET', lambdaIntegration); // You can change 'GET' to another HTTP method if needed

    // Output the API URL
    new cdk.CfnOutput(this, 'Card', {
      value: api.url,
      description: 'card API Gateway URL',
    });



  }
}

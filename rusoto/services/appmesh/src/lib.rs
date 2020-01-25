
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p>AWS App Mesh is a service mesh based on the Envoy proxy that makes it easy to monitor and
//! control microservices. App Mesh standardizes how your microservices communicate, giving you
//! end-to-end visibility and helping to ensure high availability for your applications.</p>
//!
//! <pre><code>     &lt;p&gt;App Mesh gives you consistent visibility and network traffic controls for every
//! microservice in an application. You can use App Mesh with AWS Fargate, Amazon ECS, Amazon EKS,
//! Kubernetes on AWS, and Amazon EC2.&lt;/p&gt;
//! &lt;note&gt;
//! &lt;p&gt;App Mesh supports microservice applications that use service discovery naming for their
//! components. For more information about service discovery on Amazon ECS, see &lt;a href=&quot;http://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html&quot;&gt;Service Discovery&lt;/a&gt; in the
//! &lt;i&gt;Amazon Elastic Container Service Developer Guide&lt;/i&gt;. Kubernetes &lt;code&gt;kube-dns&lt;/code&gt; and
//! &lt;code&gt;coredns&lt;/code&gt; are supported. For more information, see &lt;a href=&quot;https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/&quot;&gt;DNS
//! for Services and Pods&lt;/a&gt; in the Kubernetes documentation.&lt;/p&gt;
//! &lt;/note&gt;
//! </code></pre>
//!
//! If you're using the service, you're probably looking for [AppMeshClient](struct.AppMeshClient.html) and [AppMesh](trait.AppMesh.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;

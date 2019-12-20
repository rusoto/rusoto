#![cfg(feature = "ec2-instance-connect")]

extern crate rusoto_core;
extern crate rusoto_ec2_instance_connect;

use rusoto_core::{Region, RusotoError};
use rusoto_ec2_instance_connect::{
    Ec2InstanceConnect, Ec2InstanceConnectClient, SendSSHPublicKeyError, SendSSHPublicKeyRequest,
};

#[tokio::test]
async fn send_ssh_public_key_correctly_errors_for_unknown_instance() {
    let client = Ec2InstanceConnectClient::new(Region::UsEast1);
    let request = SendSSHPublicKeyRequest {
        availability_zone: "us-east-1a".into(),
        instance_id: "i-00000000".into(),
        instance_os_user: "ec2-user".into(),
        ssh_public_key: "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC+P68X1LG5rJXQL1ktjhMv84lP8gKgJoPk99GwWoM6lbJAv80WUgauB961I5i/3/Y0XWnYZfzFCP3+fTu/9+vEsfTd38hUW3QBGTPrx/jXyvTBRQc7bTirpeicfwL9SwM4ztYvuM45sGSeZkQIg+TMKVFGnR0ijCitG613fRP/NUw/jQjzUPj2ymCw43MIAD1BPQrznsyoaPWP/bKv91Y9ZtB1fOn3UzgWlwBGxzPNXx8boquLfHWi+ut+v1zfZpUBUjQtI4EIctjqzmxnyB1SPpxk0r5v2GR0qLChKzZ0IqdJmImlz2vqCuwUThJN9d/iF//kCeb76uJVsDOOtDWb user@host".into(),
    };

    match client.send_ssh_public_key(request).await {
        Ok(_) => {
            panic!("send_ssh_public_key should fail");
        }
        Err(error) => match error {
            RusotoError::Service(e) => match e {
                SendSSHPublicKeyError::InvalidArgs(error) => assert!(
                    error.contains("Instance not found"),
                    "Missing error message"
                ),
                _ => panic!("Unexpected error"),
            },
            _ => {
                panic!("Should have a typed error from EC2 Instance Connect");
            }
        },
    };
}

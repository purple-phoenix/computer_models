pipeline {
    agent any
    environment {
        PATH = "/var/jenkins_home/.cargo/bin:$PATH"
    }
    stages {
        stage('Build') {
            steps {
                sh 'cargo build'
            }
        }
        stage('Tests') {
            steps {
                echo "PATH is $PATH"
                sh 'cargo test'
        }
    }
}

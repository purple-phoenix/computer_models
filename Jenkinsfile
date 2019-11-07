pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'cargo build'
            }
        }
        stage('Tests') {
            steps {
                sh 'cargo test'
            }
        }
    }
}

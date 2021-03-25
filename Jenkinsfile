pipeline {
  agent any
    stages {
      stage('build') {
        steps {
          sh 'cargo build --release --all-features'
          sh 'cargo build --all-features'
        }
      }
      stage('test') {
        steps {
          sh 'cargo test'
          sh 'cargo clippy'
        }
      }
    }
}
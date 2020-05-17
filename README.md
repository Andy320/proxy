# proxy
To create a docker that router in alpine with musl supported.

In project directory:

./build.sh

cd dockerfiles

docker build --rm -f Dockerfile-router -t router:v1.0.0 .

Update build.sh and docker file to create other app with different os.

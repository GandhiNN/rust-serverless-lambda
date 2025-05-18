# build stage
FROM ghcr.io/cargo-lambda/cargo-lambda:latest AS build
ARG package

WORKDIR /build
COPY . .
RUN cargo lambda build --release

# build executables container
FROM public.ecr.aws/lambda/provided:al2023 AS runtime
ARG package

COPY --from=build /build/target/lambda/$package/bootstrap ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD ["app.handler"]
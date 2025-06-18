# TODO Lists

- [x] Create a new endpoint to keep configuration details about the subscription rule and properties to sign
- [x] The system will receive a notification with an entity data, if the configuration is not created return error 404, if the configuration was created but the list of properties is empty -> sign all the properties, otherwise sign only the property specify in the configuration.
- [ ] The generation of public-private key has to be created only once and should be connected to the IdM service
- [ ] Create the corresponding content of the README.md
- [ ] Create the dockerfile and docker image corresponding to the component

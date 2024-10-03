group "default" {
	targets = ["image"]
}

variable "TAG" {
	default = "v0.1.0"
}

target "image" {
	dockerfile = "Dockerfile"
	context = "."
    attest = [
		"type=provenance,mode=min",
		"type=sbom"
    ]
	tags = ["ci_test:${TAG}"]
}

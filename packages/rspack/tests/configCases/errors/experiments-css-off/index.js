it("should throw at runtime", () => {
  let errored = false;
	try {
		require("./index.css");
	} catch (e) {
    errored = true;
	}
  expect(errored).toBeTruthy()
});

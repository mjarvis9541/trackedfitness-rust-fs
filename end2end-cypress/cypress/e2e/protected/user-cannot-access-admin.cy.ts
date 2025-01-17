describe("ensure user cannot access admin site", () => {
  it("Navigates to home screen logs and and attempts to view admin site", () => {
    cy.visit("/");

    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.contains("a", "Admin").click();
  });
});

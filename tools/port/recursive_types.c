/* Regression input for the C2Rust DFExpr recursive-type traversal fix.
 * Translate with the pinned, patched c2rust-transpile using -std=gnu17.
 * Without a visited set, its walk of the local variable's type does not end.
 */
int recursive_types(void)
{
    struct node { struct node *next; int value; };
    struct node local = { 0, 42 };
    return local.value;
}

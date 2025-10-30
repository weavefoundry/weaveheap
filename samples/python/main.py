try:
    import weaveheap_python as wh
    print("version_major:", wh.version_major())
except Exception as e:
    print("weaveheap_python module not available (scaffold):", e)

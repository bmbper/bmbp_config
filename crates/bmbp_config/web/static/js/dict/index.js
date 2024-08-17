// web/tsx/dict/action.tsx
var PageState = {};
var PageAction = {
  init: (props) => {
    const [treeData, setTreeData] = React.useState([]);
    PageState.treeData = treeData;
    PageState.setTreeData = setTreeData;
    const [selectTreeNodeData, setSelectTreeNodeData] = React.useState();
    PageState.selectTreeNodeData = selectTreeNodeData;
    PageState.setSelectTreeNodeData = setSelectTreeNodeData;
    const [selectTreeNodeKeys, setSelectTreeNodeKeys] = React.useState([]);
    PageState.selectTreeNodeKeys = selectTreeNodeKeys;
    PageState.setSelectTreeNodeKeys = setSelectTreeNodeKeys;
    const [searchFormData, setSearchFormData] = React.useState({});
    PageState.setSearchFormData = setSearchFormData;
    PageState.searchFormData = searchFormData;
    PageState.searchFormRef = React.useRef(null);
    const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
    PageState.selectedRowKeys = selectedRowKeys;
    PageState.setSelectedRowKeys = setSelectedRowKeys;
    const [selectedRows, setSelectedRows] = React.useState([]);
    PageState.selectedRows = selectedRows;
    PageState.setSelectedRows = setSelectedRows;
    const [showPageSize, setShowPageSize] = React.useState("default");
    PageState.showPageSize = showPageSize;
    PageState.setShowPageSize = setShowPageSize;
    const [gridData, setGridData] = React.useState([]);
    PageState.gridData = gridData;
    PageState.setGridData = setGridData;
    const [pageData, setPageData] = React.useState({
      pageNo: 1,
      pageSize: 10,
      total: 0
    });
    PageState.pageData = pageData;
    PageState.setPageData = setPageData;
  },
  findTreeData: (v) => {
    if (!v || v == "") {
      PageState.setSelectTreeNodeKeys([]);
      PageState.setSelectTreeNodeData({});
    }
    console.log(v);
    const treeData = [
      {
        dictName: "\u4E00\u7EA7 1",
        dictCode: "0-0",
        dictChildren: [
          {
            dictName: "\u4E8C\u7EA7 1-0",
            dictCode: "0-0-0",
            dictChildren: [
              {
                dictName: "\u4E09\u7EA7 1-0-0",
                dictCode: "0-0-0-0"
              }
            ]
          }
        ]
      }
    ];
    PageState.setTreeData(treeData);
  },
  findGridData: (searchFormData) => {
    const gridData = [
      { dictName: "\u4E2D\u56FD", dictCode: "0-0", dictNamePath: "A/\u65F6\u4EE3\u53D1\u751F\u7684\u53D1\u751F\u5730\u65B9/sdfsfsfd/\u65AF\u8482\u82AC\u68EE\u8BE5\u6B7B\u7684\u6B4C\u5FB7\u6362\u4E2A\u5730\u65B9\u597D/,/sdfsfsfd/\u65AF\u8482\u82AC\u68EE\u8BE5\u6B7B\u7684\u6B4C\u5FB7\u6362\u4E2A\u5730\u65B9\u597D/,/sdfsfsfd/\u65AF\u8482\u82AC\u68EE\u8BE5\u6B7B\u7684\u6B4C\u5FB7\u6362\u4E2A\u5730\u65B9\u597D/", dictChildren: [{ dictName: "\u4E8C\u7EA7 1-0", dictCode: "0-0-0", dictChildren: [{ dictName: "\u4E09\u7EA7 1-0-0", dictCode: "0-0-0-0" }] }] }
    ];
    PageState.setGridData(gridData);
  },
  addBrotherNode: (node) => {
    console.log("addDict");
  },
  addChildNode: (node) => {
    console.log("addDict");
  },
  editNode: (node) => {
    console.log("editDict");
  },
  removeNode: (node) => {
    console.log("delDict");
  },
  enableNode: (node) => {
    console.log("delDict");
  },
  disableNode: (node) => {
    console.log("delDict");
  },
  changeParentNode: (node) => {
    console.log("delDict");
  },
  changeOrderNode: (node) => {
    console.log("delDict");
  }
};

// web/tsx/dict/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(PageView, null));
};
var PageView = (props) => {
  PageAction.init(props);
  React.useEffect(() => {
    PageAction.findTreeData("");
    PageAction.findGridData("");
  }, []);
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-app-fluid"
  }, /* @__PURE__ */ React.createElement(arco.Grid.Row, {
    guides: [1, 1],
    style: { height: "100vh" }
  }, /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    flex: "260px"
  }, /* @__PURE__ */ React.createElement(PageTreeView, null)), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical",
    style: { height: "100%" }
  }), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    flex: "auto",
    style: { height: "100%", width: "600px" }
  }, /* @__PURE__ */ React.createElement(PageGridView, null))));
};
var PageTreeView = () => {
  const TreeNodExtraAction = (props) => {
    const data = props.dataRef;
    return /* @__PURE__ */ React.createElement(arco.Menu, {
      style: { width: "100px", background: "#fff", border: "1px solid #e8e8e8" }
    }, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addBrother",
      onClick: () => {
        PageAction.addBrotherNode(data);
      }
    }, "\u65B0\u589E\u540C\u7EA7"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addChild",
      onClick: () => {
        PageAction.addChildNode(data);
      }
    }, "\u65B0\u589E\u5B50\u7EA7"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "edit",
      onClick: () => {
        PageAction.editNode(data);
      }
    }, "\u7F16\u8F91"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "remove",
      onClick: () => {
        PageAction.removeNode(data);
      }
    }, "\u5220\u9664"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "enable",
      onClick: () => {
        PageAction.enableNode(data);
      }
    }, "\u542F\u7528"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "disable",
      onClick: () => {
        PageAction.disableNode(data);
      }
    }, "\u505C\u7528"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "changeParent",
      onClick: () => {
        PageAction.changeParentNode(data);
      }
    }, "\u53D8\u66F4\u7236\u7EA7"));
  };
  const renderTreeNodExtra = (node) => {
    return /* @__PURE__ */ React.createElement(arco.Dropdown, {
      droplist: /* @__PURE__ */ React.createElement(TreeNodExtraAction, {
        dataRef: node.dataRef
      }),
      position: "bl"
    }, /* @__PURE__ */ React.createElement(arcoicon.IconMore, {
      style: {
        position: "absolute",
        right: 8,
        fontSize: 12,
        top: 10,
        color: "#3370ff"
      }
    }));
  };
  return /* @__PURE__ */ React.createElement("div", null, /* @__PURE__ */ React.createElement("div", {
    style: { display: "block" }
  }, /* @__PURE__ */ React.createElement(arco.Input.Search, {
    searchButton: true,
    placeholder: "\u8BF7\u8F93\u5165",
    onSearch: (v) => {
      PageAction.findTreeData(v);
    }
  })), /* @__PURE__ */ React.createElement(arco.Tree, {
    treeData: PageState.treeData,
    blockNode: true,
    renderExtra: renderTreeNodExtra,
    onSelect: (keys, extra) => {
      PageState.setSelectTreeNodeKeys(keys);
      PageState.setSelectTreeNodeData(extra.node.props.dataRef);
    },
    showLine: true,
    selectedKeys: PageState.selectTreeNodeKeys,
    fieldNames: { key: "dictCode", title: "dictName", children: "dictChildren" }
  }));
};
var PageGridView = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-container"
  }, /* @__PURE__ */ React.createElement(PageGridSearchForm, null), /* @__PURE__ */ React.createElement(PageGridToolBar, null), /* @__PURE__ */ React.createElement(PageGridTable, null), /* @__PURE__ */ React.createElement(PageGridPage, null));
};
var PageGridSearchForm = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-search"
  }, /* @__PURE__ */ React.createElement(arco.Form, {
    colon: true,
    ref: PageState.searchFormRef
  }, /* @__PURE__ */ React.createElement(arco.Grid.Row, {
    guides: [1, 1]
  }, /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 7
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u522B\u540D",
    field: "dictAlias"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u522B\u540D"
  }))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 7
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u540D\u79F0",
    field: "dictName"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u540D\u79F0"
  }))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 7
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u72B6\u6001",
    field: "dataStatus"
  }, /* @__PURE__ */ React.createElement(arco.Select, {
    placeholder: "\u8BF7\u9009\u62E9\u72B6\u6001"
  }, /* @__PURE__ */ React.createElement(arco.Select.Option, {
    key: "1",
    value: "1"
  }, "\u5DF2\u542F\u7528"), /* @__PURE__ */ React.createElement(arco.Select.Option, {
    key: "0",
    value: "0"
  }, "\u5DF2\u505C\u7528")))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 3
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, null, /* @__PURE__ */ React.createElement(arco.Space, null, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    style: { marginLeft: "8px" },
    onClick: () => {
      let fromData = PageState.searchFormRef.current.getFieldsValue();
      PageAction.findGridData(fromData);
    }
  }, "\u67E5\u8BE2"), /* @__PURE__ */ React.createElement(arco.Button, {
    onClick: () => {
      PageState.searchFormRef.current.resetFields();
    }
  }, "\u6E05\u7A7A")))))), /* @__PURE__ */ React.createElement(arco.Divider, {
    style: { margin: "0px 0 4px 0 " }
  }));
};
var PageGridToolBar = () => {
  debugger;
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-toolbar"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-toolbar major"
  }, PageState.selectTreeNodeData ? /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    onClick: () => {
      PageAction.addChildNode(PageState.selectTreeNodeData.dataRef);
    }
  }, "\u65B0\u589E") : null, PageState.selectedRowKeys && PageState.selectedRowKeys.length > 0 ? /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    status: "danger",
    onClick: () => {
      PageAction.bacthRemoveNode(PageState.selectedRowKeys);
    }
  }, "\u5220\u9664") : null), /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-toolbar extra"
  }, /* @__PURE__ */ React.createElement(arco.Button, null, "\u5BFC\u5165"), /* @__PURE__ */ React.createElement(arco.Button, null, "\u5BFC\u51FA")));
};
var PageGridTable = () => {
  const enableAction = (record) => {
    return [
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u65B0\u589E\u5B50\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        status: "danger",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconDelete, null),
        size: "mini",
        onClick: () => {
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u67E5\u770B"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconView, null),
        size: "mini",
        onClick: () => {
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u505C\u7528"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u53D8\u66F4\u4E0A\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
        }
      }))
    ];
  };
  const disableAction = (record) => {
    return [
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u7F16\u8F91"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
          PageAction.editNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u542F\u7528"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconPlayArrow, null),
        size: "mini",
        onClick: () => {
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u53D8\u66F4\u4E0A\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u65B0\u589E\u5B50\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconPlus, null),
        size: "mini",
        onClick: () => {
          PageAction.addChildNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Popconfirm, {
        focusLock: true,
        title: "\u5220\u9664\u786E\u8BA4",
        content: "\u6570\u636E\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\uFF0C\u786E\u5B9A\u5220\u9664\u5417?",
        onOk: () => {
          arco.Message.success("\u5220\u9664\u6210\u529F");
        },
        onCancel: () => {
        }
      }, /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u5220\u9664"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        status: "danger",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconDelete, null),
        size: "mini",
        onClick: () => {
          PageAction.editNode(record);
        }
      })))
    ];
  };
  const gridColumn = [
    {
      width: 200,
      ellipsis: true,
      title: "\u540D\u79F0",
      dataIndex: "dictName"
    },
    {
      width: 200,
      ellipsis: true,
      title: "\u522B\u540D",
      dataIndex: "dictAlias"
    },
    {
      width: 80,
      ellipsis: true,
      title: "\u503C",
      dataIndex: "dictValue"
    },
    {
      ellipsis: true,
      title: "\u5C42\u7EA7",
      dataIndex: "dictNamePath"
    },
    {
      title: "\u72B6\u6001",
      dataIndex: "dataStatus",
      fixed: "right",
      width: 80,
      render: (value) => {
        if (value == 1) {
          return /* @__PURE__ */ React.createElement(arco.Tag, {
            color: "green"
          }, "\u5DF2\u542F\u7528");
        } else {
          return /* @__PURE__ */ React.createElement(arco.Tag, {
            color: "red"
          }, "\u5DF2\u505C\u7528");
        }
      }
    },
    {
      title: "\u64CD\u4F5C",
      dataIndex: "action",
      width: 180,
      fixed: "right",
      align: "center",
      render: (value, record, index) => {
        return /* @__PURE__ */ React.createElement(arco.Space, null, record.dataStatus == 1 ? enableAction(record) : disableAction(record));
      }
    }
  ];
  const gridRowSelection = {
    checkAll: true,
    checkCrossPage: true,
    preserveSelectedRowKeys: true,
    fixed: true,
    columnWidth: 40,
    type: "checkbox",
    onChange: (selectedRowKeys, selectedRows) => {
      PageState.setSelectedRowKeys(selectedRowKeys);
      PageState.setSelectedRows(selectedRows);
    }
  };
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-table"
  }, /* @__PURE__ */ React.createElement(arco.Table, {
    columns: gridColumn,
    data: PageState.gridData,
    rowSelection: gridRowSelection,
    pagination: false,
    stripe: true,
    border: {
      wrapper: true,
      cell: true
    }
  }));
};
var PageGridPage = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-page"
  }, /* @__PURE__ */ React.createElement(arco.Pagination, {
    size: PageState.showPageSize,
    total: PageState.pageData.total,
    showTotal: true,
    showJumper: true,
    sizeCanChange: true,
    onChange: (pageNo, pageSize) => {
      PageState.setPageData({
        ...PageState.pageData,
        pageNo,
        pageSize
      });
    }
  }));
};

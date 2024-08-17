// web/tsx/dict/action.tsx
var PageState = {};
var PageUrl = {
  findTreeUrl: "/bmbp/config/action/dict/find_tree.action",
  findPageUrl: "/bmbp/config/action/dict/find_page.action"
};
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
    axios.post(PageUrl.findTreeUrl, { dictName: v }).then((resp) => {
      const { code, msg, data } = resp;
      if (code == 0) {
        PageState.setTreeData(data);
        PageAction.findGridData({});
      } else {
        console.log("error:", resp);
        arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
      }
    }).catch((err) => {
      console.log("error:", err);
      arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
    });
  },
  findGridData: (searchFormData) => {
    let pageParams = {
      pageNo: PageState.pageData.pageNo,
      pageSize: PageState.pageData.pageSize,
      params: {
        parentDictCode: PageState.selectTreeNodeData?.dictCode,
        ...searchFormData
      }
    };
    axios.post(PageUrl.findPageUrl, pageParams).then((resp) => {
      const { code, msg, data } = resp;
      if (code == 0) {
        PageState.setGridData(data.data);
        PageState.setPageData({ ...PageState.pageData, total: data.total });
      } else {
        console.log("error:", resp);
        arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
      }
    }).catch((err) => {
      console.log("error:", err);
      arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
    });
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
  batchRemoveNode: (keys) => {
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
  root.render(React.createElement(PageView, null));
};
var PageView = (props) => {
  PageAction.init(props);
  React.useEffect(() => {
    PageAction.findTreeData("");
    PageAction.findGridData("");
  }, []);
  return React.createElement("div", {
    className: "bmbp-app-fluid"
  }, React.createElement(arco.Grid.Row, {
    guides: [1, 1],
    style: { height: "100vh" }
  }, React.createElement(arco.Grid.Col, {
    flex: "260px"
  }, React.createElement(PageTreeView, null)), React.createElement(arco.Divider, {
    type: "vertical",
    style: { height: "100%" }
  }), React.createElement(arco.Grid.Col, {
    flex: "auto",
    style: { height: "100%", width: "600px" }
  }, React.createElement(PageGridView, null))));
};
var PageTreeView = () => {
  const TreeNodExtraAction = (props) => {
    const data = props.dataRef;
    return React.createElement(arco.Menu, {
      style: {
        width: "100px",
        background: "#fff",
        border: "1px solid #e8e8e8"
      }
    }, React.createElement(arco.Menu.Item, {
      key: "addBrother",
      onClick: () => {
        PageAction.addBrotherNode(data);
      }
    }, "\u65B0\u589E\u540C\u7EA7"), React.createElement(arco.Menu.Item, {
      key: "addChild",
      onClick: () => {
        PageAction.addChildNode(data);
      }
    }, "\u65B0\u589E\u5B50\u7EA7"), React.createElement(arco.Menu.Item, {
      key: "edit",
      onClick: () => {
        PageAction.editNode(data);
      }
    }, "\u7F16\u8F91"), React.createElement(arco.Menu.Item, {
      key: "remove",
      onClick: () => {
        PageAction.removeNode(data);
      }
    }, "\u5220\u9664"), React.createElement(arco.Menu.Item, {
      key: "enable",
      onClick: () => {
        PageAction.enableNode(data);
      }
    }, "\u542F\u7528"), React.createElement(arco.Menu.Item, {
      key: "disable",
      onClick: () => {
        PageAction.disableNode(data);
      }
    }, "\u505C\u7528"), React.createElement(arco.Menu.Item, {
      key: "changeParent",
      onClick: () => {
        PageAction.changeParentNode(data);
      }
    }, "\u53D8\u66F4\u7236\u7EA7"));
  };
  const renderTreeNodExtra = (node) => {
    return React.createElement(arco.Dropdown, {
      droplist: React.createElement(TreeNodExtraAction, {
        dataRef: node.dataRef
      }),
      position: "bl"
    }, React.createElement(arcoicon.IconMore, {
      style: {
        position: "absolute",
        right: 8,
        fontSize: 12,
        top: 10,
        color: "#3370ff"
      }
    }));
  };
  return React.createElement("div", null, React.createElement("div", {
    style: { display: "block" }
  }, React.createElement(arco.Input.Search, {
    searchButton: true,
    placeholder: "\u8BF7\u8F93\u5165",
    onSearch: (v) => {
      PageAction.findTreeData(v);
    }
  })), React.createElement(arco.Tree, {
    treeData: PageState.treeData,
    blockNode: true,
    renderExtra: renderTreeNodExtra,
    onSelect: (keys, extra) => {
      PageState.setSelectTreeNodeKeys(keys);
      PageState.setSelectTreeNodeData(extra.node.props.dataRef);
    },
    showLine: true,
    selectedKeys: PageState.selectTreeNodeKeys,
    fieldNames: {
      key: "dictCode",
      title: "dictName",
      children: "dictChildren"
    }
  }));
};
var PageGridView = () => {
  return React.createElement("div", {
    className: "bmbp-grid-container"
  }, React.createElement(PageGridSearchForm, null), React.createElement(PageGridToolBar, null), React.createElement(PageGridTable, null), React.createElement(PageGridPage, null));
};
var PageGridSearchForm = () => {
  return React.createElement("div", {
    className: "bmbp-grid-search"
  }, React.createElement(arco.Form, {
    colon: true,
    ref: PageState.searchFormRef
  }, React.createElement(arco.Grid.Row, {
    guides: [1, 1]
  }, React.createElement(arco.Grid.Col, {
    span: 7
  }, React.createElement(arco.Form.Item, {
    label: "\u522B\u540D",
    field: "dictAlias"
  }, React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u522B\u540D"
  }))), React.createElement(arco.Grid.Col, {
    span: 7
  }, React.createElement(arco.Form.Item, {
    label: "\u540D\u79F0",
    field: "dictName"
  }, React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u540D\u79F0"
  }))), React.createElement(arco.Grid.Col, {
    span: 7
  }, React.createElement(arco.Form.Item, {
    label: "\u72B6\u6001",
    field: "dataStatus"
  }, React.createElement(arco.Select, {
    placeholder: "\u8BF7\u9009\u62E9\u72B6\u6001"
  }, React.createElement(arco.Select.Option, {
    key: "1",
    value: "1"
  }, "\u5DF2\u542F\u7528"), React.createElement(arco.Select.Option, {
    key: "0",
    value: "0"
  }, "\u5DF2\u505C\u7528")))), React.createElement(arco.Grid.Col, {
    span: 3
  }, React.createElement(arco.Form.Item, null, React.createElement(arco.Space, null, React.createElement(arco.Button, {
    type: "primary",
    style: { marginLeft: "8px" },
    onClick: () => {
      let fromData = PageState.searchFormRef.current.getFieldsValue();
      PageAction.findGridData(fromData);
    }
  }, "\u67E5\u8BE2"), React.createElement(arco.Button, {
    onClick: () => {
      PageState.searchFormRef.current.resetFields();
    }
  }, "\u6E05\u7A7A")))))), React.createElement(arco.Divider, {
    style: { margin: "0px 0 4px 0 " }
  }));
};
var PageGridToolBar = () => {
  debugger;
  return React.createElement("div", {
    className: "bmbp-grid-toolbar"
  }, React.createElement("div", {
    className: "bmbp-grid-toolbar major"
  }, PageState.selectTreeNodeData ? React.createElement(arco.Button, {
    type: "primary",
    onClick: () => {
      PageAction.addChildNode(PageState.selectTreeNodeData.dataRef);
    }
  }, "\u65B0\u589E") : null, PageState.selectedRowKeys && PageState.selectedRowKeys.length > 0 ? React.createElement(arco.Button, {
    type: "primary",
    status: "danger",
    onClick: () => {
      PageAction.batchRemoveNode(PageState.selectedRowKeys);
    }
  }, "\u5220\u9664") : null), React.createElement("div", {
    className: "bmbp-grid-toolbar extra"
  }, React.createElement(arco.Button, null, "\u5BFC\u5165"), React.createElement(arco.Button, null, "\u5BFC\u51FA")));
};
var PageGridTable = () => {
  const enableAction = (record) => {
    return [
      React.createElement(arco.Tooltip, {
        content: "\u65B0\u589E\u5B50\u7EA7"
      }, React.createElement(arco.Button, {
        type: "primary",
        status: "danger",
        icon: React.createElement(arcoicon.IconDelete, null),
        size: "mini",
        onClick: () => {
        }
      })),
      React.createElement(arco.Tooltip, {
        content: "\u67E5\u770B"
      }, React.createElement(arco.Button, {
        type: "secondary",
        icon: React.createElement(arcoicon.IconView, null),
        size: "mini",
        onClick: () => {
        }
      })),
      React.createElement(arco.Tooltip, {
        content: "\u505C\u7528"
      }, React.createElement(arco.Button, {
        type: "secondary",
        icon: React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
        }
      })),
      React.createElement(arco.Tooltip, {
        content: "\u53D8\u66F4\u4E0A\u7EA7"
      }, React.createElement(arco.Button, {
        type: "secondary",
        icon: React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
        }
      }))
    ];
  };
  const disableAction = (record) => {
    return [
      React.createElement(arco.Tooltip, {
        content: "\u7F16\u8F91"
      }, React.createElement(arco.Button, {
        type: "primary",
        icon: React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
          PageAction.editNode(record);
        }
      })),
      React.createElement(arco.Tooltip, {
        content: "\u542F\u7528"
      }, React.createElement(arco.Button, {
        type: "primary",
        icon: React.createElement(arcoicon.IconPlayArrow, null),
        size: "mini",
        onClick: () => {
        }
      })),
      React.createElement(arco.Tooltip, {
        content: "\u53D8\u66F4\u4E0A\u7EA7"
      }, React.createElement(arco.Button, {
        type: "secondary",
        icon: React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
        }
      })),
      React.createElement(arco.Tooltip, {
        content: "\u65B0\u589E\u5B50\u7EA7"
      }, React.createElement(arco.Button, {
        type: "secondary",
        icon: React.createElement(arcoicon.IconPlus, null),
        size: "mini",
        onClick: () => {
          PageAction.addChildNode(record);
        }
      })),
      React.createElement(arco.Popconfirm, {
        focusLock: true,
        title: "\u5220\u9664\u786E\u8BA4",
        content: "\u6570\u636E\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\uFF0C\u786E\u5B9A\u5220\u9664\u5417?",
        onOk: () => {
          arco.Message.success("\u5220\u9664\u6210\u529F");
        },
        onCancel: () => {
        }
      }, React.createElement(arco.Tooltip, {
        content: "\u5220\u9664"
      }, React.createElement(arco.Button, {
        type: "primary",
        status: "danger",
        icon: React.createElement(arcoicon.IconDelete, null),
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
          return React.createElement(arco.Tag, {
            color: "green"
          }, "\u5DF2\u542F\u7528");
        } else {
          return React.createElement(arco.Tag, {
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
        return React.createElement(arco.Space, null, record.dataStatus == 1 ? enableAction(record) : disableAction(record));
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
  return React.createElement("div", {
    className: "bmbp-grid-table"
  }, React.createElement(arco.Table, {
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
  return React.createElement("div", {
    className: "bmbp-grid-page"
  }, React.createElement(arco.Pagination, {
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

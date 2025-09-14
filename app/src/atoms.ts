import { atom } from "jotai";
import type { Toast } from "primereact/toast";
import React from "react";

export const toastRefAtom = atom(React.createRef<Toast>());

import { atom } from "jotai";
import type { Toast } from "@mantle-ui/react/toast";
import React from "react";

export const toastRefAtom = atom(React.createRef<Toast>());

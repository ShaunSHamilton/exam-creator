import { createContext, useState, ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { ExamCreatorExam } from "@prisma/client";
import { getExamEnvironmentExam } from "../utils/fetch";

interface ExamDiffContextType {
  isDiffMode: boolean;
  setIsDiffMode: (enabled: boolean) => void;
  stagingExam: ExamCreatorExam | null | undefined;
  productionExam: ExamCreatorExam | null | undefined;
  isLoading: boolean;
  selectedEnvironment: "Staging" | "Production";
  setSelectedEnvironment: (env: "Staging" | "Production") => void;
}

export const ExamDiffContext = createContext<ExamDiffContextType | null>(null);

interface ExamDiffProviderProps {
  children: ReactNode;
  examId: string;
}

export function ExamDiffProvider({ children, examId }: ExamDiffProviderProps) {
  const [isDiffMode, setIsDiffMode] = useState(false);
  const [selectedEnvironment, setSelectedEnvironment] = useState<
    "Staging" | "Production"
  >("Staging");

  const stagingExamQuery = useQuery({
    queryKey: ["exam-environment", examId, "Staging"],
    queryFn: () =>
      getExamEnvironmentExam({
        examId,
        databaseEnvironment: "Staging",
      }),
    enabled: isDiffMode,
    retry: false,
    refetchOnWindowFocus: false,
  });

  const productionExamQuery = useQuery({
    queryKey: ["exam-environment", examId, "Production"],
    queryFn: () =>
      getExamEnvironmentExam({
        examId,
        databaseEnvironment: "Production",
      }),
    enabled: isDiffMode,
    retry: false,
    refetchOnWindowFocus: false,
  });

  const isLoading = stagingExamQuery.isPending || productionExamQuery.isPending;

  return (
    <ExamDiffContext.Provider
      value={{
        isDiffMode,
        setIsDiffMode,
        stagingExam: stagingExamQuery.data,
        productionExam: productionExamQuery.data,
        isLoading,
        selectedEnvironment,
        setSelectedEnvironment,
      }}
    >
      {children}
    </ExamDiffContext.Provider>
  );
}

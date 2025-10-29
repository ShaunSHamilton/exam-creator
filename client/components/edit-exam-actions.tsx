import {
  Box,
  Button,
  useColorModeValue,
  useDisclosure,
  useToast,
  Select,
  Text,
} from "@chakra-ui/react";
import { CodeXml, Save, GitCompare } from "lucide-react";
import {
  postValidateConfigByExamId,
  putExamById,
  putExamEnvironmentChallenges,
} from "../utils/fetch";
import {
  ExamCreatorExam,
  ExamEnvironmentChallenge,
  ExamEnvironmentConfig,
  ExamEnvironmentQuestionSet,
} from "@prisma/client";
import { useMutation } from "@tanstack/react-query";
import { GenerateModal } from "./generate-modal";
import { deserializeToPrisma } from "../utils/serde";
import { queryClient } from "../contexts";
import { useContext } from "react";
import { ExamDiffContext } from "../contexts/exam-diff";

interface EditExamActionsProps {
  exam: ExamCreatorExam;
  config: ExamEnvironmentConfig;
  questionSets: ExamEnvironmentQuestionSet[];
  examEnvironmentChallenges: Omit<ExamEnvironmentChallenge, "id">[];
}

export function EditExamActions({
  exam,
  config,
  questionSets,
  examEnvironmentChallenges,
}: EditExamActionsProps) {
  const toast = useToast();
  const {
    isOpen: generateIsOpen,
    onOpen: generateOnOpen,
    onClose: generateOnClose,
  } = useDisclosure();

  const diffContext = useContext(ExamDiffContext);

  const invalidConfigMutation = useMutation({
    mutationFn: async (examId: string) => {
      await postValidateConfigByExamId(examId);
    },
    onError(error) {
      toast({
        title: "Invalid Exam Configuration",
        description: error.message,
        status: "error",
        duration: null,
        isClosable: true,
        position: "bottom",
      });
    },
  });

  const handleDatabaseSave = useMutation({
    mutationFn: ({
      exam,
      examEnvironmentChallenges,
      config,
      questionSets,
    }: {
      exam: ExamCreatorExam;
      examEnvironmentChallenges: Omit<ExamEnvironmentChallenge, "id">[];
      config: ExamEnvironmentConfig;
      questionSets: ExamEnvironmentQuestionSet[];
    }) => {
      return Promise.all([
        putExamById({ ...exam, config, questionSets }),
        putExamEnvironmentChallenges(exam.id, examEnvironmentChallenges),
      ]);
    },
    onSuccess([examData, examEnvironmentChallengesData]) {
      // Update upstream queries cache with new data
      queryClient.setQueryData(
        ["exam", exam.id],
        deserializeToPrisma(examData)
      );
      queryClient.setQueryData(
        ["exam-challenges", exam.id],
        deserializeToPrisma(examEnvironmentChallengesData)
      );
      invalidConfigMutation.mutate(exam.id);
      toast({
        title: "Exam Saved",
        description: "Your exam has been saved to the temporary database.",
        status: "success",
        duration: 1000,
        isClosable: true,
        position: "top-right",
      });
    },
    onError(error: Error) {
      console.error(error);
      toast({
        title: "Error Saving Exam",
        description: error.message || "An error occurred saving exam.",
        status: "error",
        duration: 5000,
        isClosable: true,
        position: "top-right",
      });
    },
    retry: false,
  });

  const cardBg = useColorModeValue("gray.900", "gray.900");
  return (
    <Box
      position="fixed"
      top={3}
      right="1rem"
      zIndex={100}
      bg={cardBg}
      borderRadius="xl"
      boxShadow="lg"
      px={2}
      py={2}
      display="flex"
      flexDirection={"column"}
      alignItems="center"
      gap={4}
    >
      <Button
        leftIcon={<Save size={18} />}
        colorScheme="teal"
        variant="solid"
        px={4}
        fontWeight="bold"
        isLoading={handleDatabaseSave.isPending}
        onClick={() =>
          handleDatabaseSave.mutate({
            exam,
            config,
            questionSets,
            examEnvironmentChallenges,
          })
        }
      >
        Save to Database
      </Button>
      <Button
        leftIcon={<CodeXml size={18} />}
        colorScheme="teal"
        variant="solid"
        px={4}
        fontWeight="bold"
        onClick={generateOnOpen}
      >
        Generate Exams
      </Button>
      {diffContext && (
        <Box w="full">
          <Button
            leftIcon={<GitCompare size={18} />}
            colorScheme={diffContext.isDiffMode ? "yellow" : "teal"}
            variant={diffContext.isDiffMode ? "solid" : "outline"}
            px={4}
            fontWeight="bold"
            w="full"
            onClick={() => diffContext.setIsDiffMode(!diffContext.isDiffMode)}
          >
            {diffContext.isDiffMode ? "Hide" : "Show"} Diff
          </Button>
          {diffContext.isDiffMode && (
            <Box mt={2} w="full">
              <Text fontSize="xs" color="gray.400" mb={1}>
                Compare with:
              </Text>
              <Select
                size="sm"
                value={diffContext.selectedEnvironment}
                onChange={(e) =>
                  diffContext.setSelectedEnvironment(
                    e.target.value as "Staging" | "Production"
                  )
                }
                bg="gray.700"
                color="gray.100"
              >
                <option value="Staging">Staging</option>
                <option value="Production">Production</option>
              </Select>
            </Box>
          )}
        </Box>
      )}
      <GenerateModal
        isOpen={generateIsOpen}
        onClose={generateOnClose}
        examId={exam.id}
      />
    </Box>
  );
}
